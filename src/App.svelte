<script lang="ts">
  import { Tab, TabList, TabPanel, Tabs } from "./lib/tabs";
  import Decrypt from "./screens/Decrypt.svelte";
  import Encrypt from "./screens/Encrypt.svelte";
  import Identities from "./screens/Identities.svelte";
</script>

<main class="p-5">
  <Tabs>
    <TabList>
      <Tab>Identities</Tab>
      <Tab>Encrypt</Tab>
      <Tab>Decrypt</Tab>
    </TabList>

    <TabPanel>
      <Identities />
    </TabPanel>

    <TabPanel>
      <Encrypt />
    </TabPanel>

    <TabPanel>
      <Decrypt />
    </TabPanel>
  </Tabs>
</main>
