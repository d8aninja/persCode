from airflow import DAG
from airflow.decorators import task
from airflow.providers.databricks.operators.databricks import DatabricksSubmitRunOperator
from airflow.utils.dates import days_ago
import pandas as pd
import os
from datetime import datetime
from azure.storage.blob import BlobServiceClient

# Azure Storage Configurations
AZURE_CONNECTION_STRING = "your_azure_storage_connection_string"
AZURE_CONTAINER_NAME = "your-container-name"
AZURE_BLOB_NAME = "processed_data.parquet"

# Databricks Delta Table Location in Unity Catalog
UNITY_CATALOG_TABLE = "catalog_name.schema_name.delta_table_name"

# Define default arguments for the DAG
default_args = {
    'owner': 'airflow',
    'depends_on_past': False,
    'start_date': days_ago(1),
    'retries': 1,
}

# Define Databricks job configuration for Delta Table ingestion
databricks_delta_job_config = {
    "new_cluster": {
        "spark_version": "13.3.x-scala2.12",
        "node_type_id": "i3.xlarge",
        "num_workers": 2,
    },
    "notebook_task": {
        "notebook_path": "/Workspace/Users/example@databricks.com/LoadDeltaTable",
        "base_parameters": {
            "azure_blob_path": f"abfss://{AZURE_CONTAINER_NAME}@yourstorageaccount.dfs.core.windows.net/{AZURE_BLOB_NAME}",
            "delta_table": UNITY_CATALOG_TABLE,
        },
    },
}

# Define DAG
with DAG(
    'databricks_pipeline_with_dual_writes',
    default_args=default_args,
    schedule_interval='@daily',
    catchup=False
) as dag:

    @task
    def preprocess_data():
        """Preprocessing step: Clean the data and save as Parquet"""
        raw_data = {
            "name": ["Alice", "Bob", "Charlie", None],
            "age": [25, None, 30, 22],
            "salary": [50000, 60000, None, 45000]
        }
        df = pd.DataFrame(raw_data)

        # Fill missing values
        df.fillna({'age': df['age'].mean(), 'salary': df['salary'].median()}, inplace=True)
        df.dropna(subset=['name'], inplace=True)

        local_path = "/tmp/preprocessed_data.parquet"
        df.to_parquet(local_path, index=False, compression="snappy")

        return local_path  # Pass the file path to the next step

    @task
    def upload_to_azure(local_file_path: str):
        """Uploads Parquet file to Azure Blob Storage"""
        blob_service_client = BlobServiceClient.from_connection_string(AZURE_CONNECTION_STRING)
        blob_client = blob_service_client.get_blob_client(container=AZURE_CONTAINER_NAME, blob=AZURE_BLOB_NAME)

        with open(local_file_path, "rb") as data:
            blob_client.upload_blob(data, overwrite=True)

        azure_blob_url = f"abfss://{AZURE_CONTAINER_NAME}@yourstorageaccount.dfs.core.windows.net/{AZURE_BLOB_NAME}"
        print(f"File uploaded to Azure Blob Storage: {azure_blob_url}")

        return azure_blob_url  # Pass URL to Databricks

    @task
    def write_to_delta_table(local_file_path: str):
        """Writes data to Databricks Delta Table using Unity Catalog"""
        from pyspark.sql import SparkSession
        from delta.tables import DeltaTable

        spark = SparkSession.builder.appName("DeltaTableWrite").getOrCreate()

        # Read the Parquet file
        df = spark.read.parquet(local_file_path)

        # Append data to Delta Table
        df.write.format("delta").mode("append").saveAsTable(UNITY_CATALOG_TABLE)

        print(f"Data written to Delta Table: {UNITY_CATALOG_TABLE}")

    # Task: Run Databricks Job for Auto Loader ingestion
    run_databricks_job = DatabricksSubmitRunOperator(
        task_id='trigger_databricks_job',
        databricks_conn_id='databricks_default',
        json=databricks_delta_job_config
    )

    # Task dependencies
    file_path = preprocess_data()
    azure_url = upload_to_azure(file_path)
    write_to_delta_table(file_path)  # Writes to Delta Table
    azure_url >> run_databricks_job  # Triggers Databricks Job