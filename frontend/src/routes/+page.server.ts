import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		const name = data.get('name');
		const email = data.get('email');

		const badData = [];
		if (name === null || name === '') {
			badData.push('name');
		}
		if (email === null || email === '') {
			badData.push('email');
		}

		if (badData.length !== 0) {
			return fail(400, {
				success: false,
				message: 'Required information missing',
				badData
			});
		}

		if (
			!String(email)
				.toLowerCase()
				.match(
					/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
				)
		) {
			return fail(400, {
				success: false,
				message: 'Email was not an email',
				badData: ['email']
			});
		}

		// TODO: Send request to API

		return { success: true, message: 'Your interest has been registered, thank you!', badData };
	}
} satisfies Actions;
