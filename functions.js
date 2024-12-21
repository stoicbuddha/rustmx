import init, { handle_request } from "./pkg/rustmx.js";

export async function connectToApi() {
  // Initialize WASM and set up routing
  return init().then(() => {
    document.addEventListener("htmx:configRequest", async function (evt) {
      let path = evt.detail.path;
      const method = evt.detail.verb;
      console.log("EVENT", method, path, evt);
      // If you need raw FormData

      let params = "?";

      if (evt.detail.formData) {
        const rawFormData = evt.detail.formData;
        rawFormData.forEach((value, key) => {
          params += `${key}=${value}&`;
        });
      }
      // Remove the tail character, be it a ? or &
      params = params.slice(0, params.length - 1);
      // Stop the real request
      evt.preventDefault();

      const wasmResponse = await handle_request(path, method, params);

      window.history.pushState({}, "", path);
      // Send the fake response to htmx
      const target = evt.detail.target;
      if (target) {
        let swapStyle = "innerHTML";
        if (evt.target.hasOwnProperty("hx-swap")) {
          swapStyle = evt.target["hx-swap"];
        }
        htmx.swap(
          target, // Target element
          wasmResponse, // New HTML content
          { swapStyle } // Swap strategy (e.g., innerHTML)
        );
      }
    });

    // Handle browser back/forward
    window.addEventListener("popstate", async () => {
      console.log({ l: window.location.pathname });
      const wasmResponse = await handle_request(
        window.location.pathname,
        "GET",
        ""
      );
      let swapStyle = "innerHTML";
      htmx.swap(
        document.getElementById("content"), // Target element
        wasmResponse, // New HTML content
        { swapStyle } // Swap strategy (e.g., innerHTML)
      );
    });
  }); // Initialize WASM and intercept HTMX requests
}
