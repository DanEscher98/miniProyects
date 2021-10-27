"""
A first Flask App
"""
from flask import *

app = Flask(__name__)

@app.route('/home/admin')
def admin():
    return 'admin'

@app.route('/student')
def student():
    return 'student'


@app.route('/guest')
def guest():
    return 'guest'

@app.route('/user/<name>')
def user(name):
    if name == 'admin':
        return redirect(url_for('admin'))
    if name == 'student':
        return redirect(url_for('student'))
    if name == 'guest':
        return redirect(url_for('guest'))

@app.route('/')
def message():
    return render_template('message.html', name="User")

@app.route('/login', methods=['GET'])
def login():
    uname = request.args.get('uname')
    psswd = request.args.get('psswd')
    if uname == "dany" and psswd == "112ea5":
        return redirect(url_for('message'))

if __name__ == '__main__':    
    app.run(host="127.0.0.1",
            port=5000,
            debug=True)
