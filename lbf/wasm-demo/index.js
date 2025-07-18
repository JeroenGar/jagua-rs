import init, { initThreadPool, run_lbf_bpp_wasm, run_lbf_spp_wasm, init_logger_wasm} from "../pkg/lbf.js"; // Replace with actual wasm module name

async function loadJsonFromFileInput(fileInput) {
  return new Promise((resolve, reject) => {
    const file = fileInput.files[0];
    if (!file) {
      return reject("No file selected.");
    }
    const reader = new FileReader();
    reader.onload = () => {
      try {
        const json = JSON.parse(reader.result);
        resolve(json);
      } catch (err) {
        reject(`Invalid JSON: ${err.message}`);
      }
    };
    reader.onerror = () => reject("File read error.");
    reader.readAsText(file);
  });
}

async function run() {
  await init();
  const threads = navigator.hardwareConcurrency || 4;
  await initThreadPool(threads);
  console.log(`>> Thread Pool initialized with ${threads} threads.`);
  init_logger_wasm();

  const resultBox = document.getElementById("result");
  const svgBox = document.getElementById("svgs");

  document.getElementById("solve-btn").onclick = async () => {
    resultBox.textContent = "Running solver...";
    svgBox.innerHTML = "";

    const problemType = document.getElementById("problem-type").value;
    try {
      const instance = await loadJsonFromFileInput(document.getElementById("instance-file"));
      const config = await loadJsonFromFileInput(document.getElementById("config-file"));

      let result;
      if (problemType === "bpp") {
        result = await run_lbf_bpp_wasm(instance, config);
      } else if (problemType === "spp") {
        result = await run_lbf_spp_wasm(instance, config);
      } else {
        throw new Error("Unknown problem type.");
      }

      resultBox.textContent =
        `Solve time: ${result.solve_time_ms.toFixed(2)} ms\n\n` +
        JSON.stringify(result.output, null, 2);

      for (const [filename, svg] of result.svgs) {
        const div = document.createElement("div");
        div.innerHTML = `<h3>${filename}</h3>` + svg;
        svgBox.appendChild(div);
      }

    } catch (err) {
      resultBox.textContent = `Error: ${err}`;
      console.error(err);
    }
  };
}

run();
