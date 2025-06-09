import init, * as lbf from "./pkg/lbf.js";

// Performance tracking
const performanceData = {
  jobs: [],
  totalRustTime: 0,
  totalJSTime: 0,
  successfulJobs: 0,
  failedJobs: 0
};

// Pre-allocated log element reference
const logElement = document.getElementById('log');

// Optimized logging with fragment batching
const logBuffer = [];
let logTimeout = null;

function log(msg) {
  logBuffer.push(msg);
  
  if (logTimeout === null) {
    logTimeout = setTimeout(flushLogs, 16); // ~60fps batching
  }
}

function flushLogs() {
  if (logBuffer.length > 0) {
    logElement.textContent += logBuffer.join('\n') + '\n';
    logBuffer.length = 0; // Clear array efficiently
  }
  logTimeout = null;
}

// Update performance metrics UI
function updateMetrics() {
  const metricsContainer = document.getElementById('metricsContainer');
  metricsContainer.classList.remove('hidden');
  
  const totalJobs = performanceData.jobs.length;
  if (totalJobs === 0) return;
  
  // Calculate averages
  const avgRustTime = performanceData.totalRustTime / totalJobs;
  const avgTotalTime = performanceData.totalJSTime / totalJobs;
  const successRate = (performanceData.successfulJobs / totalJobs) * 100;
  const efficiency = totalJobs > 0 ? (performanceData.totalRustTime / performanceData.totalJSTime) * 100 : 0;
  
  // Update UI elements
  document.getElementById('rustTime').innerHTML = `${performanceData.totalRustTime.toFixed(0)}<span class="metric-unit">ms</span>`;
  document.getElementById('jsTime').innerHTML = `${performanceData.totalJSTime.toFixed(0)}<span class="metric-unit">ms</span>`;
  document.getElementById('jobCount').textContent = totalJobs;
  document.getElementById('successRate').innerHTML = `${successRate.toFixed(1)}<span class="metric-unit">%</span>`;
  document.getElementById('avgRustTime').innerHTML = `${avgRustTime.toFixed(0)}<span class="metric-unit">ms</span>`;
  document.getElementById('avgTotalTime').innerHTML = `${avgTotalTime.toFixed(0)}<span class="metric-unit">ms</span>`;
  document.getElementById('efficiency').innerHTML = `${efficiency.toFixed(1)}<span class="metric-unit">%</span>`;
}

// Clear metrics
window.clearMetrics = function() {
  performanceData.jobs = [];
  performanceData.totalRustTime = 0;
  performanceData.totalJSTime = 0;
  performanceData.successfulJobs = 0;
  performanceData.failedJobs = 0;
  
  document.getElementById('metricsContainer').classList.add('hidden');
  logElement.textContent = '';
  log('[*] Metrics cleared');
};

// Optimized object conversion with WeakMap caching and iterative approach
const conversionCache = new WeakMap();

function mapToObj(value) {
  if (value === undefined || value === null) return value;
  
  // Check cache first
  if (typeof value === 'object' && conversionCache.has(value)) {
    return conversionCache.get(value);
  }
  
  let result;
  
  if (value instanceof Map) {
    result = Object.fromEntries(
      Array.from(value.entries(), ([k, v]) => [k, mapToObj(v)])
    );
  } else if (Array.isArray(value)) {
    result = value.map(mapToObj);
  } else if (typeof value === 'object') {
    result = {};
    for (const key in value) {
      if (value.hasOwnProperty(key)) {
        result[key] = mapToObj(value[key]);
      }
    }
  } else {
    return value; // Primitive value, return as-is
  }
  
  // Cache the result
  if (typeof value === 'object') {
    conversionCache.set(value, result);
  }
  
  return result;
}

// Optimized download with object URL reuse management
const urlCache = new Set();

function download(filename, content, type = 'application/json') {
  const blob = new Blob([content], { type });
  const url = URL.createObjectURL(blob);
  
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.style.display = 'none'; // Ensure it's hidden
  
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  
  // Clean up URL after a short delay to ensure download completes
  setTimeout(() => {
    URL.revokeObjectURL(url);
    urlCache.delete(url);
  }, 100);
  
  urlCache.add(url);
}

// Separate job processing function for better error isolation
async function processJob(job) {
  const jobStart = performance.now();
  
  try {
    // Fetch instance data
    const response = await fetch(job.instancePath);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    
    const extBPInstance = await response.json();
    
    // Run the BPP solver
    const solveStart = performance.now();
    const result = await lbf.run_bpp(extBPInstance, job.inputStem);
    const solveTime = performance.now() - solveStart;
    const totalTime = performance.now() - jobStart;
    
    // Track performance data
    const jobData = {
      inputStem: job.inputStem,
      rustTime: result.solve_time_ms,
      jsTime: totalTime,
      success: true
    };
    
    performanceData.jobs.push(jobData);
    performanceData.totalRustTime += result.solve_time_ms;
    performanceData.totalJSTime += totalTime;
    performanceData.successfulJobs++;
    
    log(`[*] Job ${job.inputStem} solved in ${result.solve_time_ms}ms (Rust) / ${totalTime.toFixed(2)}ms (total)`);
    
    // Convert and download results
    const plainOutput = mapToObj(result);
    const jsonString = JSON.stringify(plainOutput, null, 2);
    download(`sol_${job.inputStem}.json`, jsonString);
    
    // Handle SVG downloads efficiently
    if (result.svgs) {
      const svgEntries = Symbol.iterator in Object(result.svgs) 
        ? Array.from(result.svgs) 
        : Object.entries(result.svgs);
      
      if (svgEntries.length > 0) {
        for (const [filename, svgContent] of svgEntries) {
          download(filename, svgContent, 'image/svg+xml');
        }
        log(`[*] Downloaded ${svgEntries.length} SVG file(s) for ${job.inputStem}`);
      } else {
        log(`[*] No SVGs found for job ${job.inputStem}`);
      }
    }
    
    return jobData;
    
  } catch (error) {
    const totalTime = performance.now() - jobStart;
    const jobData = {
      inputStem: job.inputStem,
      rustTime: 0,
      jsTime: totalTime,
      success: false,
      error: error.message
    };
    
    performanceData.jobs.push(jobData);
    performanceData.totalJSTime += totalTime;
    performanceData.failedJobs++;
    
    log(`[!] Error processing job ${job.inputStem}: ${error.message}`);
    throw error; // Re-throw for Promise.allSettled
  }
}

// Main processing function with enhanced error handling and resource management
window.run = async function() {
  const runBtn = document.getElementById('runBtn');
  const startTime = performance.now();
  
  try {
    runBtn.disabled = true;
    runBtn.textContent = 'Processing...';
    
    // Initialize WASM
    await init();
    log("[*] WASM initialized successfully!");

    // Get file input with validation
    const fileInput = document.getElementById("jobFile");
    if (!fileInput?.files?.length) {
      alert("Please upload at least one job.json file.");
      log("[!] No valid JOB found. Exiting..");
      return;
    }
    
    // Initialize thread pool with error handling
    const threads = Math.min(8, navigator.hardwareConcurrency || 4);
    try {
      await lbf.initThreadPool(threads);
      log(`[*] Thread pool initialized with ${threads} threads`);
    } catch (e) {
      log("[!] Failed to initialize thread pool. WebAssembly threading requires cross-origin isolation");
      log(`[!] Error: ${e.message}`);
      return;
    }
     
    const jobFiles = Array.from(fileInput.files);
    log(`[*] Processing ${jobFiles.length} job file(s)...`);
    
    // Pre-validate all files first to fail fast
    const validJobs = [];
    for (const file of jobFiles) {
      try {
        const jobText = await file.text();
        const job = JSON.parse(jobText);
        
        if (!job.inputStem || !job.instancePath) {
          log(`[!] Skipping ${file.name} - missing inputStem or instancePath`);
          continue;
        }
        
        validJobs.push({ file, job, jobText });
      } catch (e) {
        log(`[!] Failed to parse ${file.name}: ${e.message}`);
      }
    }
    
    if (validJobs.length === 0) {
      log("[!] No valid job files found");
      return;
    }
    
    // Process jobs with controlled concurrency to avoid overwhelming the system
    const BATCH_SIZE = Math.min(4, threads); // Process in batches
    const results = [];
    
    for (let i = 0; i < validJobs.length; i += BATCH_SIZE) {
      const batch = validJobs.slice(i, i + BATCH_SIZE);
      const batchResults = await Promise.allSettled(
        batch.map(({ job }) => processJob(job))
      );
      
      results.push(...batchResults);
      
      // Update metrics after each batch
      updateMetrics();
      
      // Small delay between batches to prevent blocking
      if (i + BATCH_SIZE < validJobs.length) {
        await new Promise(resolve => setTimeout(resolve, 10));
      }
    }
    
    // Summary
    const successful = results.filter(r => r.status === 'fulfilled').length;
    const failed = results.length - successful;
    const totalTime = performance.now() - startTime;
    
    log(`[*] Processing complete: ${successful} successful, ${failed} failed`);
    log(`[*] Total time: ${totalTime.toFixed(2)} ms`);
    log("[*] Refresh page to process more jobs");
    
    // Final metrics update
    updateMetrics();
    
    // Final log flush
    flushLogs();
    
  } catch (error) {
    log(`[!] Critical error: ${error.message}`);
    flushLogs();
  } finally {
    runBtn.disabled = false;
    runBtn.textContent = 'Run BPP';
  }
};

// Cleanup function for page unload
window.addEventListener('beforeunload', () => {
  // Clean up any remaining object URLs
  urlCache.forEach(url => URL.revokeObjectURL(url));
  flushLogs();
});
