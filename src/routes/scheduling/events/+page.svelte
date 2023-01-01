<script>
// @ts-nocheck

    let eventName = ""
    let eventDescription = ""
    let eventDate = ""
    let eventTime = ""
    let eventEnd = ""
</script>

<h1>Event Name</h1>
<input bind:value = {eventName}>
<h1>Event Description</h1>
<textarea bind:value = {eventDescription}></textarea>
<h1>Event Date</h1>
<input type="date" bind:value = {eventDate}/>
<h1>Event Time</h1>
<input type="time" id="appt" name="appt" required bind:value= {eventTime}>
<h1>Event End Time</h1>
<input type="time" id="appt" name="appt" required bind:value= {eventEnd}>