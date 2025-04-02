<script lang="ts">
	import { PieChart } from 'layerchart';
	import { statisticsState } from './state.svelte';
	import { interpolateRainbow, quantize } from 'd3';
	import PieStatistic from './PieStatistic.svelte';

	const statistics = $derived(statisticsState.statistic);
</script>

<div class="flex flex-col justify-center">
	<h1 class="text-2xl font-bold mb-4">Statistics</h1>
	<div class="stats shadow">
		<div class="stat">
			<div class="stat-title">Total Projects</div>
			<div class="stat-value">
				{#if statistics}
					{statistics.numberOfProjects}
				{:else}
					<div class="skeleton w-25"><p class="invisible">num</p></div>
				{/if}
			</div>
		</div>
		<div class="stat">
			<div class="stat-title">Languages used</div>
			<div class="stat-value">
				{#if statistics}
					{statistics.numberOfLanguages}
				{:else}
					<div class="skeleton w-25"><p class="invisible">num</p></div>
				{/if}
			</div>
		</div>
	</div>
	<div class="divider mb-0"></div>
	<p class="w-full text-center mb-3 font-semibold opacity-60 text-sm italic text-primary">
		Tip: Hover over the charts for more information
	</p>
	<div class="flex flex-wrap gap-8 justify-center">
		<PieStatistic
			data={statistics?.projectsByLanguage}
			key="language"
			value="projects"
			title="Your most used Languages"
		/>
		<PieStatistic
			data={statistics?.projectsByBuildSystem}
			key="buildSystem"
			value="projects"
			title="How you build your Projects"
		/>
		<PieStatistic
			data={statistics?.projectsByCategory}
			key="category"
			value="projects"
			title="Thats how you categorize Projects"
		/>
		<PieStatistic
			data={statistics?.projectsByIde}
			key="ide"
			value="projects"
			title="Your favourite IDEs"
		/>
	</div>
</div>
