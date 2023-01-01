<script>
    // @ts-nocheck
    import CreateTaskForm from "$lib/CreateTaskForm.svelte";
    let taskName = ""
    let taskDate = ""
    let taskTime = ""
    let taskDescription = ""


    function printTaskName(){
        console.log(taskName)
    }
</script>

<CreateTaskForm></CreateTaskForm>
