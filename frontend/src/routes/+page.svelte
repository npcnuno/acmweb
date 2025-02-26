<script lang="ts">
  import { Motion } from "svelte-motion";
  import Button from "../lib/components/ui/Button/Button.svelte";
  import Carousel from "../lib/components/ui/Carousel/Carousel.svelte";
  import TypewriterEffectSmooth from "../lib/components/ui/TypewriterEffect/TypewriterEffectSmooth.svelte";
  import { t } from "svelte-i18n"; // Added for internationalization

  const languages = [
    {
      name: "JavaScript",
      logo: "https://upload.wikimedia.org/wikipedia/commons/6/6a/JavaScript-logo.png",
    },
    {
      name: "Python",
      logo: "https://upload.wikimedia.org/wikipedia/commons/c/c3/Python-logo-notext.svg",
    },
    {
      name: "TypeScript",
      logo: "https://upload.wikimedia.org/wikipedia/commons/4/4c/Typescript_logo_2020.svg",
    },
    {
      name: "Java",
      logo: "https://upload.wikimedia.org/wikipedia/en/3/30/Java_programming_language_logo.svg",
    },
    {
      name: "C++",
      logo: "https://upload.wikimedia.org/wikipedia/commons/1/18/ISO_C%2B%2B_Logo.svg",
    },
    {
      name: "Go",
      logo: "https://upload.wikimedia.org/wikipedia/commons/0/05/Go_Logo_Blue.svg",
    },
    {
      name: "Ruby",
      logo: "https://upload.wikimedia.org/wikipedia/commons/7/73/Ruby_logo.svg",
    },
    {
      name: "Rust",
      logo: "https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg",
    },
  ];

  const projectSlides = [
    {
      id: 1,
      description: "Project One\nInnovative solution for X.",
      descriptionPosition: "bottom-center",
      image: {
        alt: "Project One",
        src: "https://via.placeholder.com/600x400?text=Project+One",
      },
    },
    {
      id: 2,
      description: "Project Two\nRevolutionizing Y.",
      descriptionPosition: "bottom-center",
      image: {
        alt: "Project Two",
        src: "https://via.placeholder.com/600x400?text=Project+Two",
      },
    },
  ];

  let blogPosts = [
    {
      id: 1,
      title: "ACM Launches New Initiative",
      excerpt: "Fostering innovation and collaboration among students.",
      date: "2025-02-24",
    },
    {
      id: 2,
      title: "Join Our Upcoming Workshop",
      excerpt: "Enhance your skills and network with professionals.",
      date: "2025-03-05",
    },
    {
      id: 3,
      title: "Hackathon 2025: Registration Open",
      excerpt: "Showcase your talent in solving real-world problems.",
      date: "2025-04-10",
    },
  ];
</script>

<div class="page-container space-y-20 px-6 py-12">
  <!-- Join Section -->
  <section class="join-section">
    <div class="container mx-auto">
      <div class="flex flex-col md:flex-row items-center gap-12">
        <Motion
          initial={{ opacity: 0, x: -100 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.8 }}
        >
          <div class="flex-1">
            <h1
              class="text-5xl font-extrabold text-blue-800 dark:text-blue-200 mb-6 leading-tight"
            >
              {$t("join.title")}
            </h1>
            <p
              class="text-xl text-gray-600 dark:text-gray-200 mb-8 leading-relaxed"
            >
              {$t("join.description")}
            </p>
            <a>
              <Button text={$t("join.button")} />
            </a>
          </div>
        </Motion>
        <Motion
          initial={{ opacity: 0, x: 100 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.8 }}
        >
          <div class="flex-1">
            <TypewriterEffectSmooth />
          </div>
        </Motion>
      </div>
    </div>
  </section>

  <!-- Blog Section -->
  <section class="blog-section">
    <div class="container mx-auto">
      <Motion
        initial={{ opacity: 0, y: 50 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
      >
        <h2
          class="text-4xl font-bold text-center text-blue-800 dark:text-blue-200 mb-12"
        >
          {$t("blog.title")}
        </h2>
        <div class="grid gap-8 md:grid-cols-3">
          {#each blogPosts as post (post.id)}
            <Motion
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              transition={{ duration: 0.5, delay: post.id * 0.2 }}
            >
              <div
                class="bg-white dark:bg-gray-800 rounded-xl shadow-md dark:shadow-gray-700 p-6 hover:shadow-xl transition-all duration-300 border border-gray-100 dark:border-gray-700"
              >
                <h3
                  class="text-2xl font-semibold text-gray-800 dark:text-gray-200 mb-3"
                >
                  {post.title}
                </h3>
                <p class="text-gray-600 dark:text-gray-300 mb-4">
                  {post.excerpt}
                </p>
                <span
                  class="text-sm text-gray-500 dark:text-gray-400 font-medium"
                  >{post.date}</span
                >
              </div>
            </Motion>
          {/each}
        </div>
      </Motion>
    </div>
  </section>

  <!-- Languages Section (Infinite Horizontal Scroll) -->
  <section class="languages-section">
    <div class="container mx-auto">
      <Motion
        initial={{ opacity: 0, y: 50 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
      >
        <h2
          class="text-4xl font-bold text-center text-blue-800 dark:text-blue-200 mb-12"
        >
          {$t("languages.title")}
        </h2>
        <div
          class="marquee-container overflow-hidden bg-gray-100 dark:bg-gray-800 py-6 rounded-xl shadow-inner"
        >
          <div class="marquee-content flex">
            {#each languages as lang (lang.name)}
              <div
                class="lang-item flex items-center space-x-4 p-4 bg-white dark:bg-gray-700 rounded-lg shadow-md hover:shadow-lg transition-shadow duration-300"
              >
                <img
                  src={lang.logo}
                  alt={lang.name}
                  class="h-12 w-12 object-contain"
                />
                <span
                  class="font-semibold text-gray-800 dark:text-gray-200 text-lg"
                  >{lang.name}</span
                >
              </div>
            {/each}
            {#each languages as lang (lang.name + "-dup")}
              <div
                class="lang-item flex items-center space-x-4 p-4 bg-white dark:bg-gray-700 rounded-lg shadow-md hover:shadow-lg transition-shadow duration-300"
              >
                <img
                  src={lang.logo}
                  alt={lang.name}
                  class="h-12 w-12 object-contain"
                />
                <span
                  class="font-semibold text-gray-800 dark:text-gray-200 text-lg"
                  >{lang.name}</span
                >
              </div>
            {/each}
          </div>
        </div>
      </Motion>
    </div>
  </section>

  <!-- Projects Section -->
  <section class="projects-section">
    <div class="container mx-auto">
      <Motion
        initial={{ opacity: 0, y: 50 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8 }}
      >
        <h2
          class="text-4xl font-bold text-center text-blue-800 dark:text-blue-200 mb-12"
        >
          {$t("projects.title")}
        </h2>
        <Carousel slides={projectSlides} />
      </Motion>
    </div>
  </section>
</div>

<style>
  .page-container {
    scroll-behavior: smooth;
    min-height: 100vh;
  }

  section {
    min-height: fit-content;
  }

  .marquee-container {
    width: 100%;
  }

  .marquee-content {
    display: flex;
    will-change: transform;
    animation: marquee 15s linear infinite;
  }

  .lang-item {
    min-width: 200px;
    flex-shrink: 0;
    margin-right: 2.5rem;
  }

  @keyframes marquee {
    0% {
      transform: translateX(0);
    }
    100% {
      transform: translateX(-50%);
    }
  }

  @media (max-width: 768px) {
    .lang-item {
      min-width: 150px;
      margin-right: 1.5rem;
    }
  }
</style>
