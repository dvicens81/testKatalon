<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getEvaluationHistoryByPatientUsingGET_1</name>
   <tag></tag>
   <elementGuidId>6b4928f2-1c9c-4282-853e-077f468aa4b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://localhost:9098/clevehr-services/api/v1/patients/${patientId}${/evaluationHistory}?limit=&amp;skip=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1b7bb646-a260-4c99-acd0-dc0e730def3b</id>
      <masked>false</masked>
      <name>patientId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
