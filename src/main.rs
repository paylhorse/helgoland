use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <main class="flex w-full min-h-screen flex-col items-center justify-between">
          <header class="sticky top-0 z-50 flex h-16 w-full shrink-0 items-center justify-between border-b border-gray-400 bg-gradient-to-r from-gray-600 to-gray-500 px-4 backdrop-blur-xl">
            <div class="flex items-center justify-center space-x-2 w-full">
              <div class="flex items-center justify-left space-x-2 w-full">
                <h1 class="text-gray-300 font-bold">{ "HELGÖLAND" }</h1>
                <button className="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-gray-400 text-primary-foreground shadow-md hover:bg-gray-500 h-8 w-12 p-0">
                  <img
                    src="/img/helgoland.png"
                    alt="Map"
                    class="h-15 w-20 shadow-md"
                  />
                </button>
              </div>
              <div class="flex items-center justify-end space-x-2 w-full">
                <div class="flex justify-end">
                  <a
                    target="_blank"
                    href="https://github.com/vercel/nextjs-ai-chatbot/"
                    rel="noopener noreferrer"
                    class="inline-flex items-center justify-center rounded-md text-sm font-medium shadow ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input border-gray-700 bg-gray-800 hover:bg-gray-900 hover:text-accent-foreground h-8 px-4 py-2 ml-2"
                  >
                    <svg
                      role="img"
                      viewBox="0 0 24 24"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="currentColor"
                      class="h-4 w-4"
                    >
                      <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"></path>
                    </svg>
                    <span class="ml-2 hidden md:flex">{ "GitHub" }</span>
                  </a>
                  <a
                    target="_blank"
                    href="https://docs.sympy.org/latest/reference/index.html#reference"
                    rel="noopener noreferrer"
                    class="inline-flex items-center justify-center rounded-md text-sm font-medium shadow ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input border-gray-600 bg-gray-700 hover:bg-gray-600 hover:text-accent-foreground h-8 px-4 py-2 ml-2"
                  >
                    <img src="img/sympyLogo.png" alt="SymPy Icon" class="h-4 w-5" />
                    <span class="ml-2 hidden md:flex">{ "SymPy Docs" }</span>
                  </a>
                  <a
                    target="_blank"
                    href="https://numpy.org/doc/stable/user/index.html"
                    rel="noopener noreferrer"
                    class="inline-flex items-center justify-center rounded-md text-sm font-medium shadow ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input border-slate-500 bg-slate-500 hover:bg-slate-400 hover:text-accent-foreground h-8 px-4 py-2 ml-2"
                  >
                    <img src="img/numpyLogo.png" alt="NumPy Icon" class="h-4 w-4" />
                    <span class="ml-2 hidden md:flex">{ "NumPy Docs" }</span>
                  </a>
                  <a
                    target="_blank"
                    href="https://matplotlib.org/stable/plot_types/index.html"
                    rel="noopener noreferrer"
                    class="inline-flex items-center justify-center rounded-md text-sm font-medium shadow ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input border-gray-500 bg-gray-400 hover:bg-gray-300 hover:text-accent-foreground h-8 px-4 py-2 ml-2"
                  >
                    <img
                      src="img/matplotlibLogo.png"
                      alt="Matplot Icon"
                      class="h-3 w-15"
                    />
                    <span class="ml-2 hidden md:flex">{ "Plot Type Docs" }</span>
                  </a>
                </div>
              </div>
            </div>
          </header>
          <div class="flex z-10 w-full items-end justify-between font-mono text-sm lg:flex">
            <div class="relative w-full h-full md:w-3/4 h-96 overflow-hidden rounded-xl border border-gray-600 bg-[#272c35] shadow-inner mr-2 ml-2 mb-4">
              <h1 class="text-gray-400 font-bold">{ "---- BLACKBOARD ----" }</h1>
              <div class="space-y-4 border-t border-gray-600 bg-gradient-to-r from-stone-400 to-stone-500 px-4 py-2 shadow-lg sm:rounded-t-xl sm:border md:py-4 mt-auto shadow-md hover:shadow-2xl">
                <form>
                  <div class="relative flex flex-col w-full px-8 grow bg-gradient-to-r from-stone-400 to-stone-500 sm:rounded-md sm:px-12 shadow-inner">
                  </div>
                  <div class="absolute right-0 top-4 sm:right-4"></div>
                </form>
                <div class="flex w-full">
                  <div class="flex justify-left items-end w-full">
                    <img
                      src="img/sympyGray.png"
                      alt="SymPy Icon"
                      class="h-4 w-5 mr-2"
                    />
                    <img
                      src="img/numpyGray.png"
                      alt="MatPlot Icon"
                      class="h-4 w-4 mr-2"
                    />
                    <img
                      src="img/matplotlibGray.png"
                      alt="MatPlot Icon"
                      class="h-3 w-15"
                    />
                  </div>
                </div>
              </div>
            </div>
              <div class="relative w-full h-full md:w-1/4 h-96 overflow-hidden rounded-xl bg-transparent shadow-inner mr-2 ml-2 mb-2">
                <h1 class="text-gray-400 font-bold">{ "ASK SCHRÖDI" }</h1>
                <div class="flex justify-between">
                  <div>
                  </div>
                </div>
                <div class="space-y-4 border-t border-gray-600 bg-gray-400 px-4 py-2 shadow-lg sm:rounded-t-xl sm:border md:py-4 mt-auto shadow-md">
                  <form>
                    <div class="relative flex flex-col w-full px-8 overflow-hidden grow bg-gray-300 sm:rounded-md sm:border sm:px-12 shadow-inner">
                    </div>
                  </form>
                  <p class="px-2 text-center text-xs leading-normal text-muted-foreground hidden sm:block text-gray-700">
                    // Powered by&nbsp;
                    <a
                      href="https://platform.openai.com/docs/introduction"
                      target="_blank"
                      class="inline-flex flex-1 justify-center gap-1 leading-4 hover:underline"
                    >
                      // <span>OpenAI</span>
                      <svg
                        aria-hidden="true"
                        height="7"
                        viewBox="0 0 6 6"
                        width="7"
                        class="opacity-70"
                      >
                        <path
                          d="M1.25215 5.54731L0.622742 4.9179L3.78169 1.75597H1.3834L1.38936 0.890915H5.27615V4.78069H4.40513L4.41109 2.38538L1.25215 5.54731Z"
                          fill="currentColor"
                        ></path>
                      </svg>
                    </a>
                  </p>
                </div>
              </div>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

