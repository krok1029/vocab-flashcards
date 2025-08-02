<script lang="ts">
  import type { DictionaryEntry } from '$lib/domain/models/dictionary';

  export let entry: DictionaryEntry;
  
  let audioPlayer: HTMLAudioElement | null = null;

  function playAudio() {
    if (audioPlayer) {
      audioPlayer.currentTime = 0;
      audioPlayer.play().catch(error => {
        console.error('Failed to play audio:', error);
      });
    }
  }
</script>

<div class="space-y-6">
  <div class="bg-gray-100 p-4 rounded shadow">
    <div class="flex items-center gap-2 mb-2">
      <h2 class="text-xl font-bold">{entry.word}</h2>
      {#if entry.audio}
        <button
          onclick={playAudio}
          class="text-sm text-blue-600 underline hover:text-blue-800 transition-colors"
          title="播放發音"
        >
          🔊 播放發音
        </button>
        <audio bind:this={audioPlayer} src={entry.audio} preload="auto"></audio>
      {/if}
    </div>

    {#if entry.phonetic}
      <p class="text-gray-600 mb-3">音標：{entry.phonetic}</p>
    {/if}

    {#each entry.meanings as meaning, index}
      <div class="mt-3" class:mt-0={index === 0}>
        <p class="font-semibold text-blue-600 mb-1">{meaning.partOfSpeech}</p>
        <ul class="list-disc list-inside ml-4 space-y-1">
          {#each meaning.definitions as def}
            <li class="text-gray-800">
              {def.definition}
              {#if def.example}
                <br />
                <small class="text-gray-500 italic">例句：{def.example}</small>
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/each}
  </div>
</div>
