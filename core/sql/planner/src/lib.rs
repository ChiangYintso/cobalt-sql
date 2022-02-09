mod plan;

use sqlparser::ast;
use sqlparser::ast::*;

pub fn plan(stmts: Vec<ast::Statement>) {
    for stmt in stmts {
        match stmt {
            ast::Statement::Commit { chain } => {}

            // DDL statements (schema changes)
            ast::Statement::CreateDatabase {
                db_name,
                if_not_exists,
                location,
                managed_location,
            } => {}
            ast::Statement::CreateTable {
                or_replace,
                temporary,
                external,
                if_not_exists,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                table_properties,
                with_options,
                file_format,
                location,
                query,
                without_rowid,
                like,
            } => {}
            ast::Statement::AlterTable { name, operation } => {}
            ast::Statement::Drop {
                object_type,
                if_exists,
                names,
                cascade,
                purge,
            } => {}

            // DML statements (mutations)
            ast::Statement::Update {
                table,
                assignments,
                selection,
            } => {}
            ast::Statement::Delete {
                table_name,
                selection,
            } => {}
            ast::Statement::Insert {
                or,
                table_name,
                columns,
                overwrite,
                source,
                partitioned,
                after_columns,
                table,
                on,
            } => {}

            // DML statements (query)
            ast::Statement::Query(query) => {}

            _ => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlparser::ast::Statement;
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    #[test]
    fn it_works() {
        for sql in [
            "CREATE TABLE foo(id int);",
            "SELECT * FROM foo JOIN bar WHERE id = 1",
            "DROP TABLE foo",
        ] {
            let ast: Vec<Statement> = Parser::parse_sql(&GenericDialect {}, sql).unwrap();
            println!("AST: {ast:#?}");
        }
    }
}
