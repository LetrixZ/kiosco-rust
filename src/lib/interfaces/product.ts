export interface Product {
	id: number;
	name: string;
	description?: string;
	barcode: string;
	price: number;
	cost: number;
	stock: number;
	created_at: string;
	updated_at: string;
}
