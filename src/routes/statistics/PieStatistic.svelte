<script lang="ts">
	import { quantize } from 'd3';
	import { interpolateRainbow } from 'd3-scale-chromatic';
	import { PieChart } from 'layerchart';

	interface Props {
		data:
			| {
					[key: string]: string | number;
			  }[]
			| undefined;
		title: string;
		key: string;
		value: string;
	}

	let { data, key, value, title }: Props = $props();
</script>

<div>
	<h3 class="font-semibold mb-2 opacity-60 text-center">{title}</h3>
	{#if data}
		<div class="h-[200px] w-[350px]">
			<PieChart
				{data}
				{key}
				{value}
				placement="left"
				cRange={quantize(interpolateRainbow, data.length)}
				legend={{ placement: 'right', orientation: 'vertical' }}
			/>
		</div>
	{:else}
		<div class="h-[200px] w-[350px] flex justify-between items-center">
			<div class="skeleton rounded-full h-[200px] w-[200px]"></div>
			<div class="skeleton h-[100px] w-[100px]"></div>
		</div>
	{/if}
</div>
