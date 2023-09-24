import returnFetch from 'return-fetch';
import { PUBLIC_SERVER_URL } from '$env/static/public';

const serverFetch = returnFetch({
	baseUrl: PUBLIC_SERVER_URL
});

export type GetCategoryResponseData = {
	id: number;
	name: string;
	createdAt: string;
};

export type GetCategoryPostsResponseData = Array<{
	id: number;
	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: string;
		name: string;
	};
	title: string;
	createdAt: string;
}>;

export type GetPostResponseData = {
	id: number;
	written_by: {
		id: number;
		email: string;
	};
	category: {
		id: string;
		name: string;
	};
	title: string;
	content: string;
	createdAt: string;
};

export type GetWriterResponseData = {
	id: number;
	email: string;
	description: string;
	createdAt: string;
	posts: Array<{
		id: number;
		title: string;
		createdAt: string;
		category: {
			id: number;
			name: string;
		};
	}>;
};

export async function getCategories(): Promise<GetCategoryResponseData[]> {
	return (await (
		await serverFetch(`/category`, {
			method: 'GET'
		})
	).json()) as GetCategoryResponseData[];
}

export async function createCategory(name: string): Promise<GetCategoryResponseData> {
	return (await (
		await serverFetch(`/category`, {
			method: 'POST',
			body: JSON.stringify({
				name
			})
		})
	).json()) as GetCategoryResponseData;
}

export async function getCategoryPosts(categoryId: number): Promise<GetCategoryPostsResponseData> {
	return (await (
		await serverFetch(`/category/${categoryId}/posts`, {
			method: 'GET'
		})
	).json()) as GetCategoryPostsResponseData;
}

export async function getPost(postId: number): Promise<GetPostResponseData> {
	return (await (
		await serverFetch(`/post/${postId}`, {
			method: 'GET'
		})
	).json()) as GetPostResponseData;
}

export async function createPost(
	categoryId: number,
	title: string,
	content: string
): Promise<GetPostResponseData> {
	return (await (
		await serverFetch(`/category/${categoryId}/posts`, {
			method: 'POST',
			body: JSON.stringify({
				title,
				content
			})
		})
	).json()) as GetPostResponseData;
}

export async function getWriters(): Promise<GetWriterResponseData[]> {
	return (await (
		await serverFetch(`/writer`, {
			method: 'GET'
		})
	).json()) as GetWriterResponseData[];
}

export async function getWriter(writerId: number): Promise<GetWriterResponseData> {
	return (await (
		await serverFetch(`/writer/${writerId}`, {
			method: 'GET'
		})
	).json()) as GetWriterResponseData;
}
