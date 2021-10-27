let hatPrice = 100;
console.log(`Hat price: ${hatPrice}`);
let bootsPrice = "100";
console.log(`Boots price: ${bootsPrice}`);

if (hatPrice === bootsPrice) {
	console.log("Prices are the same");
} else {
	console.log("Prices are different");
}

// A rest parameter is an array containing all the arguments
// for which parameters are not defined.
function sumPrices(...numbers) {
	return numbers.reduce((total, val) => {
		val = Number(val);
		return total + (Number.isNaN(val) ? 0 : val);
	}, 0);
}

let totalPrice = (sumPrices(hatPrice, bootsPrice));
console.log(`Total Price: ${totalPrice} ${typeof totalPrice}`);
totalPrice = sumPrices(1, 2, 3);
console.log(`Total Price: ${totalPrice} ${typeof totalPrice}`);
totalPrice = sumPrices(100, 200);
console.log(`Total Price: ${totalPrice} ${typeof totalPrice}`);

let myVariable = "Adam";
console.log(`Type: ${typeof myVariable}`);
myVariable = 100;
console.log(`Type: ${typeof myVariable}`);

let firstCity;
let secondCity = firstCity || "London";
console.log(`City: ${ secondCity }`);

let taxRate;
// Values that are null or undefined are converted to
// boolean false
console.log( `Tax rate: ${taxRate ?? 10}%` );
taxRate = 0;
// Bad, the OR coerces the type of zero
console.log(`Tax rate: ${taxRate || 10}%`);
// Good, the nullish coalescing operator
console.log(`Tax rate: ${taxRate ?? 10}%`);
