import type { HTMLImgAttributes, HTMLVideoAttributes } from 'svelte/elements';

export class Post {
	postDate: Date;
	year: number;
	month: number;
	day: number;
	modificationDates: Date[];
	title: string;
	content: string = 'UNINTIALIZED POST';
	author: string = 'UNINTIALIZED POST';
	headerImage: HTMLImgAttributes | undefined;
	media: (HTMLImgAttributes | HTMLVideoAttributes)[] | undefined;
	tags: string[];
	slug: string;

	constructor({
		postDate = new Date(),
		year,
		month,
		day,
		modificationDates = [],
		title = 'UNINITIALISED POST',
		author = 'UNINITIALISED POST',
		headerImage,
		media,
		tags = [],
		slug
	}: {
		postDate?: Date;
		year?: number;
		month?: number;
		day?: number;
		modificationDates?: Date[];
		title?: string;
		author?: string;
		headerImage?: HTMLImgAttributes;
		media?: (HTMLImgAttributes | HTMLVideoAttributes)[];
		tags?: string[];
		slug?: string;
	}) {
		this.postDate = postDate;

		if (postDate) {
			this.year = postDate.getFullYear();
			this.month = postDate.getMonth();
			this.day = postDate.getDay();
		} else {
			this.year = year ?? 0;
			this.month = month ?? 0;
			this.day = day ?? 0;
		}

		this.modificationDates = modificationDates;
		this.title = title;
		this.author = author;
		this.headerImage = headerImage;
		this.media = media;
		this.tags = tags;
		this.slug = slug ?? title?.replaceAll(' ', '-') ?? '';
	}
}

export class Timeline {
	#data: Record<number, Record<number, Record<number, Post[]>>> = {};

	/**
	 * Creates a new timeline and populates it with the given posts.
	 * @param posts The posts to add to the timeline.
	 */
	constructor(posts: Post[]) {
		this.populate(posts);
	}

	/**
	 * Populates the timeline with the given posts.
	 *
	 * @param posts The posts to add to the timeline.
	 */
	populate(posts: Post[]) {
		for (const post of posts) {
			const { year, month, day } = post;

			this.#data[year] = this.#data[year] || {};
			this.#data[year][month] = this.#data[year][month] || {};
			this.#data[year][month][day] = this.#data[year][month][day] || [];
			this.#data[year][month][day].push(post);
		}
	}

	/**
	 * Returns a list of years with posts.
	 *
	 * @returns List of years with posts.
	 */
	years() {
		return Object.keys(this.#data).map(Number);
	}

	/**
	 * Returns an object with the given year and a month function which returns
	 * an object with the given month and a day function which returns the posts
	 * for the given day.
	 *
	 * @param year The year for which to retrieve the posts.
	 * @returns An object with the given year and a month function which returns
	 * an object with the given month and a day function which returns the posts
	 * for the given day.
	 */
	year(year: number) {
		return {
			year
		};
	}

	/**
	 * Returns a map of months in the given year with the total amount of posts in each month.
	 *
	 * @param year The year for which to retrieve the posts.
	 * @returns A map of months to the total posts in each.
	 */
	months(year: number) {
		const yearData = this.#data[year];
		if (!yearData) return {};

		const monthsData = Object.keys(yearData).map(Number);
		const result: Record<number, number> = {};

		for (const month of monthsData) {
			const daysData = Object.keys(yearData[month]).map(Number);
			let totalPosts = 0;

			for (const day of daysData) {
				totalPosts += yearData[month][day].length;
			}

			result[month] = totalPosts;
		}

		return result;
	}

	/**
	 * Returns an object with year, month, and a day function which returns the posts for the given day.
	 *
	 * @param year The year for which to retrieve the posts.
	 * @param month The month for which to retrieve the posts.
	 * @returns An object with year, month, and a day function which returns the posts for the given day.
	 */
	month(year: number, month: number) {
		return {
			year,
			month,
			day: (day: number) => this.day(year, month, day)
		};
	}

	/**
	 * Returns a map of days in the given month with the posts for each day.
	 *
	 * @param year The year for which to retrieve the posts.
	 * @param month The month for which to retrieve the posts.
	 * @returns A map of days to the posts in each.
	 */
	days(year: number, month: number) {
		const yearData = this.#data[year];
		if (!yearData) return {};

		const monthData = yearData[month];
		if (!monthData) return {};

		const daysData = Object.keys(monthData).map(Number);
		const result: Record<number, Post[]> = {};

		for (const day of daysData) {
			result[day] = monthData[day];
		}

		return result;
	}

	/**
	 * Returns the posts for a given day.
	 * @param year The year of the day to retrieve.
	 * @param month The month of the day to retrieve.
	 * @param day The day to retrieve.
	 * @returns The posts from the given day, or an empty array if none are found.
	 */
	day(year: number, month: number, day: number) {
		return this.#data[year]?.[month]?.[day] || [];
	}

	/**
	 * Finds the index of a post in the given day.
	 * @param year The year of the post to find.
	 * @param month The month of the post to find.
	 * @param day The day of the post to find.
	 * @param post The post to find the index of.
	 * @returns The index of the post in the given day, or -1 if not found.
	 */
	findPostIndex(year: number, month: number, day: number, post: Post) {
		const dayPosts = this.day(year, month, day);
		return dayPosts.findIndex((p) => p === post);
	}

	/**
	 * Finds a post by its slug.
	 * @param slug The slug of the post to find.
	 * @returns The post with the given slug, or null if not found.
	 */
	findPostBySlug(slug: string) {
		for (const year of Object.keys(this.#data)) {
			for (const month of Object.keys(this.#data[Number(year)])) {
				for (const day of Object.keys(this.#data[Number(year)][Number(month)])) {
					const posts = this.#data[Number(year)][Number(month)][Number(day)];
					const foundPost = posts.find((post) => post.slug === slug);
					if (foundPost) {
						return foundPost;
					}
				}
			}
		}
		return null;
	}

	/**
	 * Returns the post that is chronologically next to the given post.
	 * If the given post is the last in the day, returns null.
	 * @param year The year the post is in.
	 * @param month The month the post is in.
	 * @param day The day the post is in.
	 * @param currentPost The post to find the next of.
	 * @returns The next post, or null if the given post is the last in the day.
	 */
	nextPost(year: number, month: number, day: number, currentPost: Post) {
		const dayPosts = this.day(year, month, day);
		const currentIndex = this.findPostIndex(year, month, day, currentPost);
		if (currentIndex !== -1 && currentIndex < dayPosts.length - 1) {
			return dayPosts[currentIndex + 1];
		}
		return null;
	}

	/**
	 * Returns the post that is chronologically previous to the given post.
	 * If the given post is the first in the day, returns null.
	 * @param year The year the post is in.
	 * @param month The month the post is in.
	 * @param day The day the post is in.
	 * @param currentPost The post to find the previous of.
	 * @returns The previous post, or null if the given post is the first in the day.
	 */
	previousPost(year: number, month: number, day: number, currentPost: Post) {
		const dayPosts = this.day(year, month, day);
		const currentIndex = this.findPostIndex(year, month, day, currentPost);
		if (currentIndex !== -1 && currentIndex > 0) {
			return dayPosts[currentIndex - 1];
		}
		return null;
	}

	/**
	 * Adds a post to the timeline. The post is stored in the data structure at its respective year, month, and day.
	 * If the year, month, or day does not yet exist in the data structure, it is created.
	 * @param post The post to add to the timeline.
	 */
	addPost(post: Post) {
		const { year, month, day } = post;
		this.#data[year] = this.#data[year] || {};
		this.#data[year][month] = this.#data[year][month] || {};
		this.#data[year][month][day] = this.#data[year][month][day] || [];
		this.#data[year][month][day].push(post);
	}

	/**
	 * Returns the n latest posts.
	 * @param n The number of latest posts to return.
	 * @returns The n latest posts.
	 */ getLatestPosts(n: number): Post[] {
		const allPosts: Post[] = [];

		for (const year of Object.keys(this.#data)) {
			for (const month of Object.keys(this.#data[Number(year)])) {
				for (const day of Object.keys(this.#data[Number(year)][Number(month)])) {
					allPosts.push(...this.#data[Number(year)][Number(month)][Number(day)]);
				}
			}
		}

		allPosts.sort((a, b) => b.postDate.getTime() - a.postDate.getTime());
		return allPosts.slice(0, n);
	}

	/**
	 * Returns the n oldest posts.
	 * @param n The number of oldest posts to return.
	 * @returns The n oldest posts.
	 */
	getOldestPosts(n: number): Post[] {
		const allPosts: Post[] = [];

		for (const year of Object.keys(this.#data)) {
			for (const month of Object.keys(this.#data[Number(year)])) {
				for (const day of Object.keys(this.#data[Number(year)][Number(month)])) {
					allPosts.push(...this.#data[Number(year)][Number(month)][Number(day)]);
				}
			}
		}

		allPosts.sort((a, b) => a.postDate.getTime() - b.postDate.getTime());
		return allPosts.slice(0, n);
	}
}
