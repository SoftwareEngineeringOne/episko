import commands from '$lib/commands';
import type { Statistic } from '$lib/types';

export const statisticsState = $state<{
	statistic: Statistic | null;
}>({
	statistic: null
});

export async function loadStatistics() {
	statisticsState.statistic = await commands.get_statistics();
}
