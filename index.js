import('./pkg')
  .then((m) => {
    let app = new m.FlockingApp();
    
    const canvas = document.getElementById("canvas");
    function resizeCanvas() {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }
    window.addEventListener('resize', resizeCanvas, false);
    resizeCanvas();

    function render() {
      app.draw();
      window.requestAnimationFrame(render);
    }

    render();
  })
  .catch(console.error);