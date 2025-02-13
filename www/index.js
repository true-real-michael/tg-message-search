import { Searcher } from "wasm-chat-analysis";


async function run() {
    const fileInput = document.getElementById('file-input');
    const loading = document.getElementById('loading');
    const uploadSection = document.getElementById('upload-section');
    const searchDisplaySection = document.getElementById('search-display-section');
    const threadsList = document.getElementById('threads-list');
    const detailsContent = document.getElementById('details-content');
    const searchInput = document.getElementById('search-input');

    // Mock data for threads and details (replace with actual processed data)
    let threads = [];
    let filteredThreads = [];

    // Handle file upload
    fileInput.addEventListener('change', async (event) => {
        const file = event.target.files[0];
        if (file) {
            // Show loading indicator
            loading.classList.remove('hidden');

            // Read the file as text
            const fileContent = await file.text();

            // Pass the file content to the Wasm module for processing
            const searcher = Searcher.new(fileContent);

            console.log("done");

            // // Simulate processing delay (replace with actual processing logic)
            // setTimeout(() => {
            //     // Hide loading indicator
            //     loading.classList.add('hidden');
            //
            //     // Show the search and display section
            //     uploadSection.classList.add('hidden');
            //     searchDisplaySection.classList.remove('hidden');
            //
            //     // Mock data (replace with actual processed data)
            //     threads = [
            //         {id: 1, title: "Thread 1", details: "Details for Thread 1"},
            //         {id: 2, title: "Thread 2", details: "Details for Thread 2"},
            //         {id: 3, title: "Thread 3", details: "Details for Thread 3"},
            //     ];
            //     filteredThreads = threads;
            //
            //     // Render threads
            //     renderThreads(filteredThreads);
            // }, 2000); // Simulate 2 seconds of processing
        }
    });

    // Handle search input
    searchInput.addEventListener('input', (event) => {
        const query = event.target.value.toLowerCase();
        filteredThreads = threads.filter(thread =>
            thread.title.toLowerCase().includes(query)
        );
        renderThreads(filteredThreads);
    });

    // Render threads in the left column
    function renderThreads(threads) {
        threadsList.innerHTML = threads.map(thread => `
                    <li class="p-2 hover:bg-gray-100 cursor-pointer" data-id="${thread.id}">
                        ${thread.title}
                    </li>
                `).join('');

        // Add click event listeners to threads
        document.querySelectorAll('#threads-list li').forEach(thread => {
            thread.addEventListener('click', () => {
                const threadId = thread.getAttribute('data-id');
                const selectedThread = threads.find(t => t.id == threadId);
                renderDetails(selectedThread.details);
            });
        });
    }

    // Render details in the right column
    function renderDetails(details) {
        detailsContent.innerHTML = `<p>${details}</p>`;
    }
}

run();
