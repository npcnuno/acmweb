<script lang="ts">
	export let path: string;
	export let buttonName: string;
</script>

<a class="group text-gray-900 transition-all duration-300 ease-in-out dark:text-white" href={path}>
	<span
		class="bg-gradient-to-r from-blue-700 to-blue-500 bg-[length:0%_2px] bg-left-bottom bg-no-repeat transition-all duration-500 ease-out group-hover:bg-[length:100%_2px]"
	>
		{buttonName}
	</span>
</a>
