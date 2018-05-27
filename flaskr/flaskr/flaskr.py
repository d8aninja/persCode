import os
from flask import Flask, request, sesssion, g, redirect, url_for, abort, render_template, flash

app = Flask(__name__) # create the application instance
app.config.from_object(__name__) # load the configs from this script

# load default config / override config from an env var
app.config.update(dict(
    DATABASE=os.path.join(app.root_path, 'flaskr.db'),
    SECRET_KEY='development key',
    USERNAME='admin',
    PASSWORD='default'
))
app.config.from_envvar('FLASKR_SETTINGS', silent=True)
