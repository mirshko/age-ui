<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import { Tabs, TabList, TabPanel, Tab } from "./lib/tabs";
</script>

<main class="p-5">
  <Tabs>
    <TabList>
      <Tab>Identities</Tab>
      <Tab>Encrypt</Tab>
      <Tab>Decrypt</Tab>
    </TabList>

    <TabPanel>
      <Greet />
    </TabPanel>

    <TabPanel>
      <h2>Second panel</h2>
    </TabPanel>

    <TabPanel>
      <h2>Third panel</h2>
    </TabPanel>
  </Tabs>
</main>
