import init, * as lbf from "./pkg/lbf.js";

function log(msg) {
  const el = document.getElementById('log');
  el.textContent += msg + '\n';
}

function mapToObj(value) {
  if (value === undefined || value === null) return value;

  if (value instanceof Map) {
    const obj = {};
    for (const [k, v] of value.entries()) {
      obj[k] = mapToObj(v);
    }
    return obj;
  } else if (Array.isArray(value)) {
    return value.map(mapToObj);
  } else if (typeof value === 'object') {
    const obj = {};
    for (const key in value) {
      if (Object.prototype.hasOwnProperty.call(value, key)) {
        obj[key] = mapToObj(value[key]);
      }
    }
    return obj;
  } else {
    return value;
  }
}

function download(filename, content, type = 'application/json') {
  const blob = new Blob([content], { type });
  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = filename;
  a.click();
  URL.revokeObjectURL(a.href);
}

async function run() {
  await init(); // Ensure Wasm is initialized
  const fileInput = document.getElementById("jobFile");
  if (!fileInput.files.length) {
    alert("Please upload a job.json file.");
    return;
  }

  const jobText = await fileInput.files[0].text();
  const job = JSON.parse(jobText);

  const { inputStem, instancePath, outputDir } = job;

  if (!inputStem || !instancePath) {
    log("[!] Missing 'inputStem' or 'instancePath'");
    return;
  }

  // Try to fetch input JSON
  let extBPInstance;
  try {
    const res = await fetch(instancePath);
    extBPInstance = await res.json();
  } catch (e) {
    log(`[!] Failed to fetch input from ${instancePath}`);
    return;
  }

  try {
    const result = await lbf.run_bpp(extBPInstance, inputStem);
    const resultObj = JSON.parse(JSON.stringify(result));

    log(`[*] Wasm returned result in (ms): ${resultObj.solve_time_ms}`);

    const plainOutput = mapToObj(result);
    const jsonString = JSON.stringify(plainOutput, null, 2);
    download(`sol_${inputStem}.json`, jsonString);

    if (result.svgs && Symbol.iterator in Object(result.svgs)) {
      for (const [filename, svgContent] of result.svgs) {
        download(filename, svgContent, 'image/svg+xml');
      }
    } else {
      log("[*] No SVGs found to save.");
    }
  } catch (e) {
    log(`[!] Error: ${e}`);
  }
}

window.run = run;
