<script lang="ts"> 
  
  import "../../../app.pcss" 

  export let name;

</script>


<div id="separator" class="flex justify-start items-center p-3 text-sm font-bold text-gray-300">

  { name }

</div>


<div id="separator-minimized" class="bg-gray-300"></div>

<style>

  #separator {

    width: --side-bar-width;
    height: 30px;
    margin-top: 8px;
    display: var( --side-bar-normal-screen-display-flex );

  }

  #separator-minimized {

    height: 0.5px;
    margin:auto;
    margin-top: 8px;
    width: 70px;
    display: var( --side-bar-small-screen-display );

  }

</style>
