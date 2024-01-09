<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { homeDir } from "@tauri-apps/api/path";
  import { open } from "@tauri-apps/api/shell";
  import Bar from "./lib/Bar.svelte";

  let currentDir: string = "";
  let fileList = ["../", "hola/", "pene.exe", "ajsdj.pdf"];
  let showHidden = false;

  document.addEventListener("keypress", (e) => {
    if (e.key == "h") {
      showHidden = !showHidden
      updateFileList(currentDir)
    }
  })

  async function updateFileList(parentDir: string) {
    fileList = await invoke<string[]>("get_files", {
      parentDir,
      showHidden
    });
  }

  (async function () {
    currentDir = await homeDir();
    updateFileList(currentDir);
  })();

  async function handleClick(selectedFile: string) {
    if (!selectedFile.endsWith("/")) {
      open(currentDir + selectedFile);
      return;
    }


    if (selectedFile == "../") {
      let lastSlashIndex = currentDir.lastIndexOf("/", currentDir.length - 2);
      currentDir = `${currentDir.substring(0, lastSlashIndex)}/`;
    } else {
      currentDir += selectedFile;
    }

    await updateFileList(currentDir);
    document.documentElement.scrollTop = 0;
  }
</script>

<main id="fileList">
  {#each fileList as file}
    <button
      type="button"
      class:dir={file.endsWith("/") && file != "../"}
      on:click={async () => handleClick(file)}>{file}</button
    >
  {/each}
</main>
<Bar currentDir={currentDir} showHidden={showHidden} />

<style>
  :global(#bar) {
    position: fixed;
    bottom: 0;
  }

  #fileList button {
    border: 0;
    border-bottom: 1px solid var(--overlay0);
    display: block;
    padding: 1em 1ch;
    width: 100%;
    &:hover {
      background-color: var(--surface0);
    }
    &:last-child {
      border-bottom: 0;
      margin-bottom: 1.6rem;
    }
  }

  .dir {
    color: var(--blue);
    font-weight: 700;
  }
</style>
