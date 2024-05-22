import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: './src/schemas/magasin.graphqls',
	documents: [
		'./src/lib/magasin/**/*.svelte',
		'src/lib/magasin/**/*.ts',
		'src/routes/magasin/**/*.svelte',
		'src/routes/magasin/**/*.ts'
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		'./src/lib/magasin/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
