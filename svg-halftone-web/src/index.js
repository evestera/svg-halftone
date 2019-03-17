import("../../svg-halftone-wasm/pkg").then(module => {
  const render = () => {
    const input = document.getElementById('input');
    const files = input.files;
    if (files.length == 0) {
      return;
    }
    const file = files[0];
    document.getElementById('inputname').innerHTML = file.name;

    const outputwidthInput = document.getElementById('outputwidth');
    const outputwidth = parseFloat(outputwidthInput.value);
    if (Number.isNaN(outputwidth) || outputwidth === 0.0) {
      return;
    }

    const spacingInput = document.getElementById('spacing');
    const spacing = parseFloat(spacingInput.value);
    if (Number.isNaN(spacing) || spacing === 0.0) {
      return;
    }

    const invert = !document.getElementById('invert').checked;
    const cutpaths = document.getElementById('cutpaths').checked;

    const spinnerOverlay = document.getElementById('spinner-overlay');
    spinnerOverlay.className = '';

    const reader = new FileReader();
    reader.onload = () => {
      const buffer = reader.result;
      const arr = new Uint8Array(buffer);
      const result = module.run(
        arr,
        outputwidth,
        spacing,
        document.getElementById('shape').value,
        document.getElementById('grid').value,
        invert,
        cutpaths
      );
      const target = document.getElementById('target');
      target.innerHTML = result || '<p>Something went wrong when generating the SVG. Sorry!</p>';
      spinnerOverlay.className = 'is-invisible';

      const svgDoc = '<?xml version="1.0" encoding="UTF-8" standalone="no"?>' + result;
      const encoded = btoa(svgDoc);
      const downloadLink = document.getElementById('download');
      downloadLink.href = 'data:application/octet-stream;base64,' + encoded;
    };
    reader.readAsArrayBuffer(file);
  };

  document.getElementById('input').onchange = render;
  document.getElementById('outputwidth').onchange = render;
  document.getElementById('spacing').onchange = render;
  document.getElementById('shape').onchange = render;
  document.getElementById('grid').onchange = render;
  document.getElementById('invert').onchange = render;
  document.getElementById('cutpaths').onchange = render;
});
