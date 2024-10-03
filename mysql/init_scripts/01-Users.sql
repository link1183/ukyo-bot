-- Admin user
CREATE USER 'admin'@'%' IDENTIFIED BY 'nFo4nAs53?jSJAnS';
GRANT ALL PRIVILEGES ON *.* TO 'admin_user'@'%' WITH GRANT OPTION;

-- Exporter user with specific permissions
CREATE USER 'exporter'@'localhost' IDENTIFIED BY 'exportneptpk' WITH MAX_USER_CONNECTIONS 3;
GRANT PROCESS, REPLICATION CLIENT, SELECT ON *.* TO 'exporter'@'localhost';

-- Apply changes
FLUSH PRIVILEGES;
