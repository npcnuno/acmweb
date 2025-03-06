<script>
  import Markdown from "svelte-markdown";
  import Prism from "prismjs";
  import "prismjs/components/prism-javascript"; // Add languages you need
  import "prismjs/components/prism-css";

  export let post;
  export async function load({ params }) {
    const { slug } = params;
    try {
      post = await getPost(slug);
      if (post) {
        return { props: { post } };
      } else {
        return { status: 404, error: new Error("Post not found") };
      }
    } catch (error) {
      return { status: 500, error };
    }
  }
  const options = {
    highlight: (code, lang) => {
      const language = Prism.languages[lang] ? lang : "text";
      return Prism.highlight(code, Prism.languages[language], language);
    },
  };
</script>

<div class="prose dark:prose-invert">
  <Markdown source={post.content} {options} />
</div>
