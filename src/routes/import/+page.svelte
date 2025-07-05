<script lang="ts">
  import { Input } from '$lib/components/ui/input/index.js';
  let files: FileList;
  let fileContent = '';
  let lines: string[] = [];

  $: if (files && files.length > 0) {
    const file = files[0];
    const reader = new FileReader();
    reader.onload = (e) => {
      fileContent = e.target?.result as string;
      let rawLines = fileContent
        .split('\n')
        .map((l) => l.trim().toLowerCase())
        .filter((l) => l.length > 0);

      let uniqueWords = [...new Set(rawLines)];
      lines = uniqueWords;
    };
    reader.readAsText(file);
  }
</script>

<div class="p-4 space-y-4">
  <h1 class="text-2xl font-bold">📥 匯入 .txt 檔案</h1>
  <div class="my-4 text-lg text-gray-500">
    <a href="/">Go to search page</a>
  </div>
  <Input type="file" accept=".txt" bind:files />

  {#if lines.length > 0}
    <h2 class="text-lg font-semibold">預覽內容（{lines.length} 筆）</h2>
    <ul class="bg-gray-50 p-4 rounded max-h-[300px] overflow-y-auto border">
      {#each lines as line, i}
        <li class="py-1 border-b text-sm text-gray-800">{i + 1}. {line}</li>
      {/each}
    </ul>
  {/if}
</div>
