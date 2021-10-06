import csv
import os


class Switcher(object):
    def indirect(self,i):
        method_name = 'op_'+str(i)
        method = getattr(self, method_name, lambda:'Invalid')
        return method()
    def op_1(self):
        input("1- ")

def menu():
    print("1- Manage accounts")
    print("2- Do movements")
    print("3- Statistics")
    n = int(input("Choose one: "))
    Switcher().indirect(n)


#%%
if __name__=='__main__':
    user = 1
    while(user == 1):
        os.system("clear")
        menu()
        user = int(input("Repeat program?: "))
    os.system("clear")