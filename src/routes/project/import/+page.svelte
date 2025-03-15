<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';

	let pickFilePromise = pickFile();

	async function pickFile() {
		return await open({
			multiple: false,
			directory: false
		});
	}
</script>

<h1>Import manifest file</h1>
{#await pickFilePromise}
	Pick a file...
{:then file}
	Picked ${file}
{:catch error}
	Failed to pick file: ${error}
{/await}
