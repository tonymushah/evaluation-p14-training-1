<script lang="ts" context="module">
	import { v4 } from 'uuid';
	import { faker } from '@faker-js/faker';
	import lodash from 'lodash';
	import { readable, type Readable } from 'svelte/store';
	import { createTable, Render, Subscribe } from 'svelte-headless-table';
	import * as Table from '$lib/components/ui/table';

	export type RefLaptopTableItem = {
		id: string;
		nom: string;
		processeur: string;
		carteGraphique: string;
		ecran: number;
		clavier: string;
		prix: number;
	};
	function genFakeData(number: number): RefLaptopTableItem[] {
		const data: RefLaptopTableItem[] = [];
		for (let index = 0; index < number; index++) {
			data.push({
				id: v4(),
				nom: faker.commerce.productName(),
				prix: Number(faker.commerce.price()),
				carteGraphique: faker.commerce.productMaterial(),
				processeur: faker.commerce.productAdjective(),
				ecran: lodash.random(9, 12),
				clavier: faker.commerce.productName()
			});
		}
		return data;
	}
	const testData: RefLaptopTableItem[] = genFakeData(lodash.random(10, 20));
</script>

<script lang="ts">
	export let data: Readable<RefLaptopTableItem[]> = readable(testData);
	const table = createTable(data);
	const columns = table.createColumns([
		table.column({
			accessor: 'id',
			header: 'ID'
		}),
		table.column({
			accessor: 'nom',
			header: 'processeur'
		}),
		table.column({
			accessor: 'carteGraphique',
			header: 'Carte Graphique'
		}),
		table.column({
			accessor: 'ecran',
			header: 'Ecran'
		}),
		table.column({
			accessor: 'clavier',
			header: 'Clavier'
		}),
		table.column({
			accessor: 'prix',
			header: 'Prix'
		})
	]);
	const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);
</script>

<div class="rounded-md border">
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowAttrs={headerRow.attrs()}>
					<Table.Row>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
								<Table.Head {...attrs}>
									<Render of={cell.render()} />
								</Table.Head>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row {...rowAttrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									<Render of={cell.render()} />
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
</div>
