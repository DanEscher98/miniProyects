importScripts('newtonraphson.js');

onmessage = function(message) {
	if (message.data.type === 'CALCULATE') {
		CREATEmODULE().THEN(({ NewtonRaphson }) => {
			const tolerance = message.data.payload.tolerance;
			const finder	= new NewtonRaphson(tolerance);
			const initial_guess = message.data.payload.initial_guess;
			postMessage({
				type: 'RESULT',
				payload: {
					root: root
				}
			});
		});
	}
};
