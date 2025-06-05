import * as lbf from "./pkg/lbf.js";
import * as fs from 'fs/promises';
import path from 'path';

// Recursively convert Maps to plain objects
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

async function downloadFile(outputDir, filename, content) {
  if (content === undefined) {
    console.error(`[!] Content for ${filename} is undefined!`);
    return;
  }
  try {
    const outputPath = path.resolve(outputDir, filename);
    await fs.mkdir(path.dirname(outputPath), { recursive: true });
    await fs.writeFile(outputPath, content);
    console.log(`[+] Saved: ${outputPath}`);
  } catch (err) {
    console.error(`[!] Failed to save ${filename}:`, err);
  }
}

async function main() {
  try {
    const job = JSON.parse(await fs.readFile('./job/job.json', 'utf-8'));

    const { inputStem, instancePath, outputDir } = job;

    if (!inputStem || !instancePath || !outputDir) {
      throw new Error("Missing 'inputStem', 'instancePath', or 'outputDir' in job.json");
    }

    const extBPInstance = JSON.parse(await fs.readFile(instancePath, 'utf-8'));

    const result = await lbf.run_bpp(extBPInstance, inputStem);
    console.log("[*] Wasm returned in(ms): ", result.solve_time_ms);

    const plainOutput = mapToObj(result);
    const jsonString = JSON.stringify(plainOutput, null, 2);
    await downloadFile(outputDir, `sol_${inputStem}.json`, jsonString);

    if (result.svgs && Symbol.iterator in Object(result.svgs)) {
      for (const [filename, svgContent] of result.svgs) {
        await downloadFile(outputDir, filename, svgContent);
      }
    } else {
      console.log("[*] No SVGs found to save.");
    }
  } catch (e) {
    console.error("[!] Error:", e);
  }
}

main();
