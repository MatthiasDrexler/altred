import adapter from "@sveltejs/adapter-node"
import preprocess from "svelte-preprocess"

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [preprocess()],

  kit: {
    // hydrate the <div id="svelte"> element in src/app.html
    target: "#svelte",

    adapter: adapter({
      // default options are shown
      out: "target",
      precompress: true,
      env: {
        host: "HOSTS",
        port: "PORT",
      },
    }),
  },
}

export default config
