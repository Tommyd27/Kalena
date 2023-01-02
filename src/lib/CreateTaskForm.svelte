<script>
    import { invoke } from '@tauri-apps/api/tauri';

// @ts-nocheck
    import { Input, Label, Helper, Textarea, Button, Select} from 'flowbite-svelte';
    let name = ""
    let to_do_by = ""
    let to_do_after = ""
    let type = ""
    let description = ""
    let repeats = 0
    let until = ""
    let per = ""
    let frequency = 0
    class Task {
		constructor(name, to_do_by, to_do_after, type, description, repeats, until, per, frequency) {
			this.name = name
            this.to_do_by = to_do_by
            this.to_do_after = to_do_after
            this.task_type = type
            this.description = description
            this.repeats = repeats
            this.until = until
            this.per = per
            this.frequency = frequency
		}
	}
    let taskTypes = [
        {value : "normal", name : "Normal"},
        {value : "repeated", name : "Repeated"},
    ]
    let perOptions = [
        {value : "day", name : "Day"},
        {value : "week", name : "Week"},
        {value : "month", name : "Month"},
    ]
    let textareaprops = {
        id: 'description',
        name: 'description',
        label: 'Task Description',
        rows: 4,
        placeholder: 'Description',
    };

    async function onSubmit(e) {
        console.log(e)
        let task = new Task(name, to_do_by, to_do_after, type, description, repeats, until, per, frequency)
        console.log("hello", task)
        let result = await invoke("create_task", {task})
        console.log(result)
        if (result) {
            e.target.reset();
        }
        else {
            //error
        }
    }
</script>

<form style = "margin-left: 5px" on:submit|preventDefault={onSubmit}>
    <div class="grid gap-6 mb-6 md:grid-cols-2">
      <div>
        <Label for="task_name" class="mb-2">Task name</Label>
        <Input type="text" id="task_name" placeholder="Daily Leetcode" required bind:value = {name}/>
      </div>
      <div>
        <Label for="task_type" class="mb-2">Task Type</Label>
        <Select class="mt-2" items={taskTypes} bind:value={type} required/>
      </div>
      {#if type == "normal"}
        <div>
            <Label for="to-do-by" class="mb-2">To Do By</Label>
            <Input type="date" id="to-do-by" required bind:value = {to_do_by}/>
        </div>
        <div>
            <Label for="available-by" class="mb-2">Available After</Label>
            <Input type="date" id="available-by" bind:value = {to_do_after}/>
        </div>
      {:else}
        <div>
            <Label for="repeats" class="mb-2">Repeats</Label>
            <Input type="number" id="repeats" placeholder="0" bind:value = {repeats}/>
        </div>
        <div>
            <Label for="until" class="mb-2">Until</Label>
            <Input type="date" id="until" bind:value = {until}/>
        </div>
        <div>
            <Label for="freq" class="mb-2">Frequency</Label>
            <Input type="number" id="freq" placeholder="0" bind:value = {frequency}/>
        </div>
        <div>
            <Label for="per" class="mb-2">Per</Label>
            <Select class="mt-2" items={perOptions} bind:value={per} required/>
        </div>
      {/if}
    </div>
    <div class="mb-6">
        <Textarea {...textareaprops} required bind:value = {description}/>
    </div>
    <Button type="submit">Submit</Button>
  </form>