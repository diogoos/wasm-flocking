import('./pkg')
  .then((m) => {
    // CANVAS RESIZING
    const canvas = document.getElementById("canvas");
    function resizeCanvas() {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }
    window.addEventListener('resize', resizeCanvas, false);
    resizeCanvas();

    // WASM RENDERING
    let app = new m.FlockingApp();
    function render() {
      app.draw();
      window.requestAnimationFrame(render);
    }
    render();

    // SETTINGS
    document.getElementById("triangleboid").checked = true;
    document.getElementById("showgrid").checked = false;
    document.getElementById("triangleboid").onchange = function () { app.show_boid_headings = this.checked; }
    document.getElementById("showgrid").onchange = function () { app.show_qtree_grid = this.checked; }
  })
  .catch(console.error);