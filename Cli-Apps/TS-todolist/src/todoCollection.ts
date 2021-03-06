import { TodoItem } from "./todoItem";

// type ItemCounts = {
// 	total: number,
// 	incomplete: number
// }

export class TodoCollection {
	private nextId: number = 1;
	// Use of geneneric object Map
	protected itemMap = new Map<number, TodoItem>();
	constructor(public userName: string,
				public todoItems: TodoItem[] = []) {
		// Initialize the associative array
		todoItems.forEach((item) => this.itemMap.set(item.id, item));
	}
	
	addTodo(task: string): number {
		while (this.getTodoById(this.nextId)) {
			this.nextId++;
		}
		this.todoItems.push(new TodoItem(this.nextId, task));
		this.itemMap.set(this.nextId, new TodoItem(this.nextId, task));
		return this.nextId;
	}

	getTodoById(id: number) : TodoItem {
		return this.itemMap.get(id);
	}
	
	getTodoItems(includeComplete: boolean): TodoItem[] {
		return Array.from(this.itemMap.values())
			.filter((item) => includeComplete || !item.complete);
	}

	markComplete(id: number,
				 complete: boolean) {
		const todoItem = this.getTodoById(id);
		if (todoItem) {
			todoItem.complete = complete;
		}
	}

	removeCompleted() {
		this.itemMap.forEach(item => {
			if (item.complete) {
				this.itemMap.delete(item.id);
			}
		})
	}

	getItemCounts(): {total: number, incomplete: number} {
		return {
			total: this.itemMap.size,
			incomplete: this.getTodoItems(false).length
		};
	}
}
