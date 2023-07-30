export function nonAnimationInterval(callback, ms) {
    const controller = new AbortController();
    const {signal} = controller;
    let handle = -1;
    let lastInstant = Date.now();
    handle = setInterval(() => {
        if (signal.aborted) {
            clearInterval(handle);
            return;
        }
        callback(Date.now() - lastInstant);
        lastInstant = Date.now();
    }, ms);
    return controller;
}

export function animationInterval(callback, ms) {
    const controller = new AbortController();
    const {signal} = controller;

    // Prefer currentTime, as it'll better sync animtions queued in the 
    // same frame, but if it isn't supported, performance.now() is fine.
    const start = document.timeline ? document.timeline.currentTime : performance.now();

    let lastInstant = start;    

    function frame(time) {
      if (signal.aborted) return;
      callback(time - lastInstant);
      lastInstant = time;
      scheduleFrame(time);
    }
  
    function scheduleFrame(time) {
      const elapsed = time - start;
      const roundedElapsed = Math.round(elapsed / ms) * ms;
      const targetNext = start + roundedElapsed + ms;
      const delay = targetNext - performance.now();
      setTimeout(() => requestAnimationFrame(frame), delay);
    }
  
    scheduleFrame(start);
    return controller;
}