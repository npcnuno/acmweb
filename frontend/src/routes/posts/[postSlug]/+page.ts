/** @type {import('./$types').PageLoad} */
export async function load({ fetch, url, params }) {
	const data = await fetch(`${url.origin}/api/posts/${params.postSlug}`);
	const res = await data.json();
	return res;
}
