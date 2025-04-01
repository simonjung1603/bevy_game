export default function myInitializer () {
  return {
    onStart: () => {
      console.log("Loading...");
      console.time("trunk-initializer");
    },
    onProgress: ({current, total}) => {
      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        console.log("Loading...", Math.round((current/total) * 100), "%" )
      }
    },
    onComplete: () => {
      console.log("Loading... done!");
      console.timeEnd("trunk-initializer");
    },
    onSuccess: (wasm) => {
      console.log("Loading... successful!");
      console.log("WebAssembly: ", wasm);
      
      const progressBarLabel = document.getElementById("progress-bar-label");
      const progressBar = document.getElementById("progress-bar");
      const startButton = document.getElementById("start-button");

      loadingBarLabel.style.display = "none";
      loadingBar.style.display = "none";
      startButton.attributes.removeNamedItem("disabled");
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    }
  }
};
