import sqlite3

def connect_db():
    """Connects to a specific database."""
    rv = sqlite3.connect(app.config['DATABASE'])
    rv.row_factory(sqlite3.Row)
    return rv

def get_db():
    """Opens a new db connection if one doesn't exist for the current application context"""
    if not hasattr(g, 'sqlite_db'):
        g.sqlite_db = connect_db()
    return sqlite_db

@app.teardown_appcontext
def close_db(error):
    """Closes the database again at the end of the request"""
    if hasattr(g, 'sqlite_db'):
        g.sqlite_db.close()

