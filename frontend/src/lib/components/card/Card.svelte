<script>
  export let title = "Card Title";
  export let text = "This is some text within the card.";
  export let imageUrl = "https://via.placeholder.com/150";
  export let buttonText = "Learn More";
</script>

<div class="max-w-sm rounded overflow-hidden shadow-lg bg-white">
  <div class="px-6 py-4">
    <div class="font-bold text-xl mb-2">{title}</div>
    <p class="text-gray-700 text-base">
      {text}
    </p>
  </div>
  <div class="px-6 pt-4 pb-2">
    <button
      class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    >
      {buttonText}
    </button>
  </div>
</div>
