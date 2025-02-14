import { Searcher } from "wasm-chat-searcher";

async function run() {
    const fileInput = document.getElementById("file-input");
    const loading = document.getElementById("loading");
    const uploadSection = document.getElementById("upload-section");
    const searchDisplaySection = document.getElementById(
        "search-display-section",
    );
    const threadsList = document.getElementById("threads-list");
    const detailsContent = document.getElementById("details-content");
    const searchInput = document.getElementById("search-input");
    const searchButton = document.getElementById("search-button");

    let searcher = null;
    let fileContent = null;

    // 1. Initialize Searcher in the background
    const initSearcher = async () => {
        searcher = Searcher.new();
        if (fileContent) {
            searcher.set_data(fileContent);
            fileInput.classList.add("hidden");
            loading.classList.add("hidden");
            uploadSection.classList.add("hidden");
            searchDisplaySection.classList.remove("hidden");
        }
    };

    initSearcher();

    // 2. Handle file upload
    fileInput.addEventListener("change", async (event) => {
        const file = event.target.files[0];
        if (file) {
            // Show loading indicator
            loading.classList.remove("hidden");

            // Read the file as text
            fileContent = await file.text();

            // 3. Set data if searcher is already initialized
            if (searcher) {
                searcher.set_data(fileContent);
                fileInput.classList.add("hidden");
                loading.classList.add("hidden");
                uploadSection.classList.add("hidden");
                searchDisplaySection.classList.remove("hidden");
            }
        }
    });

    // Function to update threads based on the search query
    function updateThreads() {
        const query = searchInput.value.toLowerCase();
        const threads = searcher.find_threads(query);
        renderThreads(threads);
    }

    // Handle search button click
    searchButton.addEventListener("click", () => {
        updateThreads();
    });

    // Handle Enter key press in the search input
    searchInput.addEventListener("keypress", (event) => {
        if (event.key === "Enter") {
            updateThreads();
        }
    });

    // Render threads in the left column
    function renderThreads(threads) {
        threadsList.innerHTML = threads
            .map(
                (thread) => `
                    <li class="p-2 hover:bg-gray-100 cursor-pointer" data-id="${thread.thread_id}">
                        ${thread.title_text}
                    </li>
                `,
            )
            .join("");

        // Add click event listeners to threads
        document.querySelectorAll("#threads-list li").forEach((thread) => {
            thread.addEventListener("click", () => {
                const threadId = thread.getAttribute("data-id");
                const selectedThread = threads.find((t) => t.thread_id == threadId);
                renderDetails(searcher.get_thread_messages(selectedThread.thread_id));
            });
        });
    }

    // Render details in the right column
    function renderDetails(details) {
        detailsContent.innerHTML = details
            .map(
                (message) => `
        <div class="bg-blue-100 rounded-lg p-2 m-2">
            ${
                    message.reply_to_text
                        ? `
                <div class="truncate bg-blue-200 p-1 ml-1">
                    ${message.reply_to_text}
                </div>
            `
                        : ""
                }
            <div>${message.text}</div>
        </div>
    `,
            )
            .join("");
    }
}

run();
