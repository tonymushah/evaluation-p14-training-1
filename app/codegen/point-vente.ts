import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
	schema: './src/schemas/point-vente.graphqls',
	documents: [
		'./src/lib/point-vente/**/*.svelte',
		'src/lib/point-vente/**/*.ts',
		'src/routes/point-vente/**/*.svelte',
		'src/routes/point-vente/**/*.ts'
	],
	ignoreNoDocuments: true, // for better experience with the watcher
	generates: {
		'./src/lib/point-vente/gql/': {
			preset: 'client',
			config: {
				useTypeImports: true
			},
			plugins: []
		}
	}
};

export default config;
