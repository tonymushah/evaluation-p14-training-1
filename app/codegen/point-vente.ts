import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: './app/schemas/point-vente.graphqls',
	documents: [
		'app/lib/point-vente/**/*.svelte',
		'app/lib/point-vente/**/*.ts',
		'app/routes/point-vente/**/*.svelte',
		'app/routes/point-vente/**/*.ts'
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		'./app/lib/point-vente/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
