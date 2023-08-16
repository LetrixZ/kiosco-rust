import fs from 'fs';

fs.readFile('db.sql', 'utf8', (err, data) => {
	if (err) {
		console.error('Error reading the file:', err);
		return;
	}

	// Split the SQL statements by semicolon
	const sqlStatements = data.split(';');

	// Remove empty statements and whitespace
	const validSqlStatements = sqlStatements.filter((statement) => statement.trim() !== '');

	// Format and modify the last two columns of each statement
	const modifiedSqlStatements = validSqlStatements.map((statement) => {
		const modifiedStatement = statement.replace(/(\d+\.\d+),(\d+\.\d+)/g, (match, price, cost) => {
			const modifiedPrice = (parseFloat(price) * 100).toFixed(0);
			const modifiedCost = (parseFloat(cost) * 100).toFixed(0);
			return `${modifiedPrice},${modifiedCost}`;
		});

		return modifiedStatement;
	});

	// Join the modified statements back into a single string
	const formattedSql = modifiedSqlStatements.join(';\n') + ';';

	// Write the modified SQL to a new file
	const outputFilePath = 'formatted.sql';
	fs.writeFile(outputFilePath, formattedSql, 'utf8', (err) => {
		if (err) {
			console.error('Error writing the file:', err);
		} else {
			console.log('Formatted SQL has been written to:', outputFilePath);
		}
	});
});
