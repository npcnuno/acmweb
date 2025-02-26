<script>
  // Define props with default values
  export let title = "Card Title";
  export let text = "This is some text within the card.";
  export let imageUrl = "https://via.placeholder.com/150";
  export let buttonText = "Learn More";
</script>

<div
  class="w-full rounded-lg overflow-hidden shadow-lg bg-white dark:bg-gray-800 transform hover:scale-105 transition-transform duration-300"
>
  <!-- Image at the top -->
  <img src={imageUrl} alt="Card image" class="w-full h-48 object-cover" />

  <!-- Content section -->
  <div class="px-6 py-4">
    <div class="font-bold text-2xl mb-4 text-gray-900 dark:text-white">
      {title}
    </div>
    <p class="text-gray-700 dark:text-gray-300 text-base">
      {text}
    </p>
  </div>

  <!-- Button section -->
  <div class="px-6 pt-4 pb-4">
    <button
      class="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg w-full"
    >
      {buttonText}
    </button>
  </div>
</div>
