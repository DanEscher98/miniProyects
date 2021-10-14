import { TodoItem } from "./todoItem"
import { TodoCollection } from "./todoCollection"
import * as inquirer from 'inquirer';
// import { JsonTodoCollection } from "./jsonTodoCollection";

function displayTodoList(collection: TodoCollection) : void {
	console.log(`${collection.userName}\'s Todo List `
				+ `(${collection.getItemCounts().incomplete} items to do)`);
	collection.getTodoItems(showCompleted).forEach((item) => 
												   item.printDetails());
}

function promptAdd(collection: TodoCollection) : void {
	inquirer.prompt({
		type: "input",
		name: "add",
		message: "Enter task:"
	}).then((answers) => {
		if (answers["add"] != "") {
			collection.addTodo(answers["add"]);
		}
		promptUser();
	})
}

function promptComplete(collection: TodoCollection) : void {
	inquirer.prompt({ type: "checkbox",
					name: "complete",
					message: "Mark Tasks Completed",
					choices: collection.getTodoItems(showCompleted)
						.map((item) => ({name: item.task,
										value: item.id,
										checked: item.complete}))
	}).then((answers) => {
		// 'as' is a type assertion, it overrides the compiler
		let completedTasks = answers["complete"] as number[];
		collection.getTodoItems(true)
		// for each item, checks that it exists
			.forEach((item) =>
					 collection.markComplete(item.id,
						completedTasks.find((id) =>
											id === item.id) != undefined
		));
		promptUser();
	})
}

// Not point free functions

function promptUser(): void {
	enum Commands {
		Add		= "Add a New Task",
		Complete= "Complete a Task",
		Toggle	= "Show/Hide Completed",
		Purge	= "Remove Completed Tasks",
		Quit	= "Quit",
	}

	console.clear();
	displayTodoList(collection);
	inquirer.prompt({
		type: "list",
		name: "command",
		message: "Choose option",
		choices: Object.values(Commands),
		// badProperty: true
	}).then((answers) => {
		switch (answers["command"]) {
			case Commands.Toggle:
				showCompleted = !showCompleted;
				promptUser();
				break;
			case Commands.Add:
				promptAdd(collection);
				break;
			case Commands.Complete:
				if (collection.getItemCounts().incomplete > 0) {
					promptComplete(collection);
				} else {
					promptUser();
				} break;
			case Commands.Purge:
				collection.removeCompleted();
				promptUser();
				break;
		}
	})
}

// Program script
let todos = [
	new TodoItem(1, "Buy Flowers", true),
	new TodoItem(2, "Get Shoes"),
	new TodoItem(3, "Collect Tickets"), 
	new	TodoItem(4, "Call Joe", true)
];

let showCompleted = true;
let collection: TodoCollection = new TodoCollection("Daniel", todos);
collection.addTodo("Go for run");
collection.addTodo("Do the laundry")
promptUser();
// console.log(JSON.stringify(todoItem));
