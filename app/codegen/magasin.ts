import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: './app/schemas/magasin.graphqls',
	documents: [
		'app/lib/magasin/**/*.svelte',
		'app/lib/magasin/**/*.ts',
		'app/routes/magasin/**/*.svelte',
		'app/routes/magasin/**/*.ts'
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		'app/lib/magasin/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
