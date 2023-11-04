
```mermaid
classDiagram
      Task <|-- Pending
      Task <|-- Done
      Task : +String title
      Task : +TaskStatus status
      class Pending{
          +new(String) -> Pending
		  +get()
		  +create()
		  +edit()
      }
      class Done{
          +new(String) -> Done
		  +get()
		  +edit()
		  +delete()
      }
	  TaskStatus : +enum [PENDING, DONE]
      TaskStatus : +stringify()
	  TaskStatus : +from_string(String)
      TaskStatus : +fmtDisplay()
	  Traits: +get()
	  Traits: +create()
	  Traits: +edit()
	  Traits: +delete()
```



ref: https://support.typora.io/Draw-Diagrams-With-Markdown/