<script lang="ts">
  let fileContent: string = '';
  let lines: string[] = [];

  function handleFileUpload(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (e) => {
      fileContent = e.target?.result as string;
      let rawLines = fileContent
        .split('\n')
        .map((l) => l.trim().toLowerCase())
        .filter((l) => l.length > 0);

      let uniqueWords = [...new Set(rawLines)];
      lines = uniqueWords.map((word) => {
        return word;
      });
    };
    reader.readAsText(file);
  }
</script>

<div class="p-6 space-y-4">
  <h1 class="text-2xl font-bold">📥 匯入 .txt 檔案</h1>

  <input type="file" accept=".txt" on:change={handleFileUpload} />

  {#if lines.length > 0}
    <h2 class="text-lg font-semibold">預覽內容（{lines.length} 筆）</h2>
    <ul class="bg-gray-50 p-4 rounded max-h-[300px] overflow-y-auto border">
      {#each lines as line, i}
        <li class="py-1 border-b text-sm text-gray-800">{i + 1}. {line}</li>
      {/each}
    </ul>
  {/if}
</div>
