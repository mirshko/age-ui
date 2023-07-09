<script>
  import { getContext } from "svelte";
  import { TABS } from "./Tabs.svelte";

  const tab = {};
  const { registerTab, selectTab, selectedTab } = getContext(TABS);

  registerTab(tab);
</script>

<button
  on:click={() => selectTab(tab)}
  class={`relative min-w-0 flex-1 overflow-hidden bg-white py-4 px-4 text-center text-sm font-medium hover:bg-gray-50 focus:z-10 select-none ${
    $selectedTab === tab ? "text-gray-900" : "text-gray-500 hover:text-gray-700"
  }} first:rounded-l-lg last:rounded-r-lg`}
  aria-current={$selectedTab === tab ? "page" : "false"}
>
  <slot />
  <span
    aria-hidden="true"
    class={$selectedTab === tab
      ? "bg-indigo-500 absolute inset-x-0 bottom-0 h-0.5"
      : "bg-transparent absolute inset-x-0 bottom-0 h-0.5"}
  />
</button>
